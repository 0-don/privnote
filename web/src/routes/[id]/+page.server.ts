import type { Note, ResponseBody } from '$lib/@types';
import type { Actions, Redirect, RequestEvent } from '@sveltejs/kit';
import { redirect } from '@sveltejs/kit';

import { client, getCsrfToken, messageFactory } from '$lib/utils/server';
import { deepMerge } from '$lib/utils';
import { COOKIE } from '$lib/utils/server/constants';
import { DeleteNoteSchema } from '$lib/schemas/deleteNote.schema';
import type { PageServerLoad } from '../$types';

export const load = (async (options: RequestEvent): Promise<ResponseBody | Redirect | void> => {
  if (!options.params?.id) throw redirect(307, '/');

  try {
    const body = await (
      await client<ResponseBody<{ note: Note; alert: string }>>(`note/${options.params.id}`, {
        method: 'GET'
      })
    ).body.json();

    const captcha = await getCsrfToken(options);

    return deepMerge(body, captcha, body.data?.note ? { messages: messageFactory(body.data?.note) } : {});
  } catch (err) {
    // throw redirect(307, '/');
    return { messages: [{ message: JSON.stringify(err), path: 'error' }] };
  }
}) satisfies PageServerLoad;

export const actions = {
  password: async (options) => {
    const form = Object.fromEntries(await options.request.formData());

    try {
      const text = options.cookies.get(COOKIE);
      const tag = form.tag;

      const body = await (
        await client<ResponseBody<{ note: Note; alert: string }>>(
          `note/${options.params.id}?manual_password=${form.manual_password}&tag=${tag}&text=${text}`,
          { method: 'GET' }
        )
      ).body.json();

      return deepMerge(body, body.data?.note ? { messages: messageFactory(body.data?.note) } : {});
    } catch (err) {
      return { messages: [{ message: JSON.stringify(err), path: 'error' }, ...messageFactory(form)] };
    }
  },
  delete: async ({ request, cookies }): Promise<ResponseBody | Redirect> => {
    const form = Object.fromEntries(await request.formData());
    try {
      const text = cookies.get(COOKIE);

      const body = JSON.stringify(DeleteNoteSchema.parse({ ...form, text }));

      const res = await (
        await client<ResponseBody<boolean>>(`note`, {
          method: 'DELETE',
          body
        })
      ).body.json();

      return res;
    } catch (err) {
      return {
        messages: [
          {
            message: JSON.stringify(err),
            path: 'error'
          },
          ...messageFactory(form)
        ]
      } as ResponseBody;
    }
  }
} satisfies Actions;
