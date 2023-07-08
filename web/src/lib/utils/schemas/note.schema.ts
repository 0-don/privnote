import { z } from 'zod';

export const NoteSchema = z.object({
  note: z.string().min(1).max(10000),
  duration_hours: z.coerce.number().int().min(0).max(24).optional(),
  manual_password: z.string().min(0).max(100).optional(),
  manual_password_confirm: z.string().min(0).max(100).optional(),
  notify_email: z.string().email().trim().toLowerCase().min(0).max(100).optional().or(z.literal('')),
  notify_ref: z.string().min(0).max(100).optional()
});
