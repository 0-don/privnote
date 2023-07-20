export type PathKeys =
  | 'tag'
  | 'note'
  | 'duration_hours'
  | 'manual_password'
  | 'manual_password_confirm'
  | 'destroy_without_confirmation'
  | 'notify_email'
  | 'notify_ref'
  | 'data'
  | 'error'
  | 'ok';
export type Note = {
  id: string;
  note: string;
  duration_hours: number;
  manual_password: string;
  notify_email: string;
  created_at: string;
  deleted_at: string;
};

export type JSONValue = string | number | boolean | { [x: string]: JSONValue } | Array<JSONValue>;

export type ResponseBody<T = {}> = {
  data?: JSONValue;
  messages?: Messages[];
} & {
  data?: T;
  messages?: Messages[];
};

export type Messages = {
  message: JSONValue;
  path: PathKeys;
  key?: string;
};

export type DebugMessages = {
  form?: Messages[];
  captcha?: Messages[];
  data?: Messages[];
};

export type Text = { text: string };
export type Tag = { tag: string };
export type Captcha = Tag & Text;

export type NoteResponse = {
  note: Note;
  alert: string;
};
