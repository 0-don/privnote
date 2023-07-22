import { z } from 'zod';

export const DeleteNoteSchema = z.object({
  text: z.string().min(1).max(10000),
  tag: z.coerce.number().int().min(0).max(255),
  id: z.string().min(17).max(17),

  manual_password: z.string().optional().nullable().or(z.literal(null)).default(null)
});
