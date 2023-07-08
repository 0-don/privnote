export type NoteKeys =
  | 'note'
  | 'duration_hours'
  | 'manual_password'
  | 'manual_password_confirm'
  | 'notify_email'
  | 'notify_ref'
  | 'error'
  | 'ok';

export type CreateNoteResponse = {
  message: string;
  path: NoteKeys;
};
