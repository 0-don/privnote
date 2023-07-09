export type NoteKeys =
  | 'id'
  | 'note'
  | 'duration_hours'
  | 'manual_password'
  | 'manual_password_confirm'
  | 'destroy_without_confirmation'
  | 'notify_email'
  | 'notify_ref'
  | 'error'
  | 'ok';

export type Notification = {
  message: string;
  path: NoteKeys;
};

export type CaptchLoad = { id: string } | Notification[];
