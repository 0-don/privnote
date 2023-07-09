export type NoteKeys =
  | 'tag'
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

export type Captcha = {
  text: string;
  tag: number;
};

export type CaptchaLoad = { tag: string } | Notification[];
