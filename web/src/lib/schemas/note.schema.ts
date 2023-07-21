import { z } from 'zod';

export const NoteSchema = z
  .object({
    tag: z.coerce.number().int().min(0).max(255),
    note: z.string().min(1).max(100000),

    destroy_without_confirmation: z.enum(['1', '0']).transform((v) => v === '1'),
    duration_hours: z.coerce.number().int().min(0).max(720).optional(),
    manual_password: z.string().min(1).max(100).optional().or(z.literal('')),
    manual_password_confirm: z.string().min(1).max(100).optional().or(z.literal('')),
    notify_email: z.string().email().trim().toLowerCase().min(0).max(100).optional().or(z.literal('')),
    delete_at: z.string().optional().nullable().or(z.literal(null)).default(null)
  })
  .superRefine(({ manual_password, manual_password_confirm }, ctx) => {
    if (manual_password !== manual_password_confirm) {
      ctx.addIssue({
        code: 'custom',
        path: ['manual_password_confirm'],
        message: 'The passwords did not match'
      });
    }
  });
