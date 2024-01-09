import { z } from "zod"

export const DeckboxtopiaCardSchema = z.object({ name: z.string(), art_url: z.string().url() });

export type DeckboxtopiaCard = z.infer<typeof DeckboxtopiaCardSchema>;