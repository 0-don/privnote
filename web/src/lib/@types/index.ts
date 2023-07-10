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

export type Captcha = Tag & Text;

export type Text = { text: string };

export type Tag = { tag: string };

export type Key = { key: string };

export type CaptchaLoad = Tag | Notification[];

export type NotificationEvent = Notification & Key;
