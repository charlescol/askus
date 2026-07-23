import { promises as fs } from 'node:fs';
import * as path from 'node:path';
import { Injectable, Logger } from '@nestjs/common';
import { ConfigService } from '@nestjs/config';
import OpenAI from 'openai/index.js';

const TIME_ZONE = 'Europe/Paris';
const PEOPLE = ['Charles', 'Thomas', 'Romain', 'Victor', 'Benjamin'] as const;
const THEMES = [
  '😂 Drôle / Absurde',
  '🌶️ Pimenté / Dossiers',
  '🧠 Personnalité / Qui nous connaît vraiment ?',
  '🏆 Futur / Qui est le plus susceptible de…',
  '🔥 Situations / Choix impossibles',
] as const;

type Theme = (typeof THEMES)[number];

interface PollHistoryItem {
  date: string;
  theme: Theme;
  question: string;
}

interface TelegramResponse {
  ok: boolean;
  description?: string;
}

const GENERATION_INSTRUCTIONS = `
Tu écris le sondage quotidien d'un groupe Telegram privé composé de cinq amis :
Charles, Thomas, Romain, Victor et Benjamin.

Crée UNE question originale, courte, immédiatement compréhensible et en français.
Elle doit donner envie de voter puis de débattre, concerner directement le groupe,
et permettre de choisir naturellement une seule personne parmi les cinq.

Le ton peut être drôle, absurde, révélateur ou légèrement piquant, mais jamais
méchant, humiliant, discriminatoire, sexuel de manière explicite, ni susceptible
de provoquer un conflit sérieux. Préfère une situation concrète à une question
générique. Varie la formulation et ne commence pas systématiquement par
« Qui est le plus susceptible de… ».

Réponds uniquement avec la question : pas de préfixe, pas de guillemets, pas
d'explication, pas de thème, pas d'options et pas de retour à la ligne. La question
doit se terminer par un point d'interrogation et faire au maximum 300 caractères.
`.trim();

@Injectable()
export class DailyPollService {
  private readonly logger = new Logger(DailyPollService.name);
  private readonly openai: OpenAI;
  private readonly openaiModel: string;
  private readonly telegramBotToken: string;
  private readonly telegramChatId: string;
  private readonly historyFile: string;

  constructor(private readonly config: ConfigService) {
    this.openai = new OpenAI({
      apiKey: this.config.getOrThrow<string>('OPENAI_API_KEY'),
      timeout: 30_000,
      maxRetries: 2,
    });
    this.openaiModel = this.config.get<string>('OPENAI_MODEL', 'gpt-5.6-luna');
    this.telegramBotToken = this.config.getOrThrow<string>('TELEGRAM_BOT_TOKEN');
    this.telegramChatId = this.config.getOrThrow<string>('TELEGRAM_CHAT_ID');
    this.historyFile = this.config.get<string>(
      'DAILY_POLL_HISTORY_FILE',
      path.resolve(process.cwd(), 'data', 'daily-polls.json'),
    );
  }

  async publishDailyPoll(): Promise<void> {
    const history = await this.readHistory();
    const theme = this.themeForToday();

    try {
      const question = await this.generateQuestion(theme, history);
      await this.sendTelegramPoll(question);
      try {
        await this.writeHistory([...history, { date: this.dateInParis(), theme, question }]);
      } catch {
        this.logger.warn('The poll has been published, but its history could not be recorded.');
      }
      this.logger.log(`Daily poll published (${theme}).`);
    } catch (error) {
      const message = error instanceof Error ? error.stack : String(error);
      this.logger.error('Failed to publish the daily poll.', message);
    }
  }

  private async generateQuestion(theme: Theme, history: PollHistoryItem[]): Promise<string> {
    const recentQuestions = history.slice(-30).map(({ question }) => question);
    let rejectionReason = '';

    for (let attempt = 1; attempt <= 2; attempt += 1) {
      const response = await this.openai.responses.create({
        model: this.openaiModel,
        instructions: GENERATION_INSTRUCTIONS,
        input: [
          `Thème imposé aujourd'hui : ${theme}.`,
          'Questions récentes à ne pas répéter ni reformuler de trop près :',
          recentQuestions.length > 0 ? recentQuestions.map((question) => `- ${question}`).join('\n') : 'Aucune.',
          rejectionReason,
        ]
          .filter(Boolean)
          .join('\n\n'),
        max_output_tokens: 160,
      });

      const question = this.cleanQuestion(response.output_text);
      rejectionReason = this.validateQuestion(question, recentQuestions);

      if (!rejectionReason) {
        return question;
      }

      rejectionReason = `La proposition précédente a été refusée : ${rejectionReason}. Génère une autre question.`;
    }

    throw new Error('OpenAI has not produced a valid question in two attempts.');
  }

  private cleanQuestion(value: string): string {
    return value
      .trim()
      .replace(/^question\s*:\s*/iu, '')
      .replace(/^[«“"]|[»”"]$/gu, '')
      .trim();
  }

  private validateQuestion(question: string, recentQuestions: string[]): string {
    if (!question) return 'la réponse est vide';
    if (question.length > 300) return 'elle dépasse 300 caractères';
    if (question.includes('\n')) return 'elle contient plusieurs lignes';
    if (!question.endsWith('?')) return 'elle ne se termine pas par ?';

    const normalized = this.normalize(question);
    if (recentQuestions.some((recent) => this.normalize(recent) === normalized)) {
      return 'elle a déjà été publiée';
    }

    return '';
  }

  private async sendTelegramPoll(question: string): Promise<void> {
    const response = await fetch(`https://api.telegram.org/bot${this.telegramBotToken}/sendPoll`, {
      method: 'POST',
      headers: { 'content-type': 'application/json' },
      body: JSON.stringify({
        chat_id: this.telegramChatId,
        question,
        options: PEOPLE.map((text) => ({ text })),
        type: 'regular',
        is_anonymous: true,
        allows_multiple_answers: false,
      }),
      signal: AbortSignal.timeout(15_000),
    });

    const payload = (await response.json()) as TelegramResponse;
    if (!response.ok || !payload.ok) {
      throw new Error(`Telegram has refused the poll : ${payload.description ?? response.status}`);
    }
  }

  private themeForToday(): Theme {
    const [year, month, day] = this.datePartsInParis();
    const dayNumber = Math.floor(Date.UTC(year, month - 1, day) / 86_400_000);
    return THEMES[((dayNumber % THEMES.length) + THEMES.length) % THEMES.length];
  }

  private dateInParis(): string {
    const [year, month, day] = this.datePartsInParis();
    return [year, month, day].map((part, index) => String(part).padStart(index === 0 ? 4 : 2, '0')).join('-');
  }

  private datePartsInParis(): [number, number, number] {
    const parts = new Intl.DateTimeFormat('fr-FR', {
      timeZone: TIME_ZONE,
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
    }).formatToParts(new Date());

    const value = (type: Intl.DateTimeFormatPartTypes) => Number(parts.find((part) => part.type === type)?.value);

    return [value('year'), value('month'), value('day')];
  }

  private normalize(value: string): string {
    return value
      .normalize('NFD')
      .replace(/\p{Diacritic}/gu, '')
      .toLocaleLowerCase('fr-FR')
      .replace(/[^a-z0-9]+/g, ' ')
      .trim();
  }

  private async readHistory(): Promise<PollHistoryItem[]> {
    try {
      const content = await fs.readFile(this.historyFile, 'utf8');
      const parsed: unknown = JSON.parse(content);
      if (!Array.isArray(parsed)) return [];

      return parsed
        .filter(
          (item): item is PollHistoryItem =>
            typeof item === 'object' &&
            item !== null &&
            typeof item.date === 'string' &&
            typeof item.question === 'string' &&
            THEMES.includes(item.theme as Theme),
        )
        .slice(-60);
    } catch (error) {
      if ((error as NodeJS.ErrnoException).code === 'ENOENT') return [];
      this.logger.warn('History is ill-formed ; starting with an empty history.');
      return [];
    }
  }

  private async writeHistory(history: PollHistoryItem[]): Promise<void> {
    await fs.mkdir(path.dirname(this.historyFile), { recursive: true });
    const temporaryFile = `${this.historyFile}.tmp`;
    await fs.writeFile(temporaryFile, `${JSON.stringify(history.slice(-60), null, 2)}\n`, 'utf8');
    await fs.rename(temporaryFile, this.historyFile);
  }
}
