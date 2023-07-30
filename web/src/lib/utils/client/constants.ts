import type { ResponseBody, Messages } from '$lib/@types';

export const debugLog = (data: ResponseBody, form: ResponseBody) => ({
  data: [
    ...(data?.data ? [{ key: 'response', path: 'data', message: data?.data }] as any : []),
    ...(form?.data ? [{ key: 'request', path: 'form', message: form?.data }] : [])
  ] as Messages[],
  form: (form?.messages || []).map((m) => ({ ...m, key: 'form' })),
  captcha: (data?.messages || []).map((m) => ({ ...m, key: 'captcha' }))
});
