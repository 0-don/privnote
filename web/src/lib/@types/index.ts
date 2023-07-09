import { text } from '@sveltejs/kit';

export type NoteKeys =
  | 'note'
  | 'duration_hours'
  | 'manual_password'
  | 'manual_password_confirm'
  | 'destroy_without_confirmation'
  | 'notify_email'
  | 'notify_ref'
  | 'error'
  | 'ok';

export type CreateNoteResponse = {
  message: string;
  path: NoteKeys;
};

export type CaptchLoad = { captcha: string } | CreateNoteResponse;
