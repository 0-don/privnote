import type { Note, ResponseBody } from '$lib/@types';
import type { Actions, Redirect } from '@sveltejs/kit';
import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { client, getCaptcha } from '$lib/utils/server';
import { deepMerge } from '$lib/utils';
import { COOKIE } from '$lib/utils/server/constants';
import { DeleteNoteSchema } from '$lib/schemas/deleteNote.schema';

export const load = (async (options): Promise<ResponseBody | Redirect | void> => {
  if (!options.params?.id) throw redirect(307, '/');

  try {
    const body = await (
      await client<ResponseBody<{ note: Note; alert: string }>>(`note/${options.params.id}`, {
        method: 'GET'
      })
    ).body.json();

    const captcha = await getCaptcha(options);

    return deepMerge(body, captcha);
  } catch (err) {
    // throw redirect(307, '/');
    return { messages: [{ message: JSON.stringify(err), path: 'error' }] };
  }
}) satisfies PageServerLoad;

export const actions = {
  default: async ({ request, cookies }): Promise<ResponseBody | Redirect> => {
    try {
      const text = cookies.get(COOKIE);

      const form = Object.fromEntries(await request.formData());

      const body = JSON.stringify(DeleteNoteSchema.parse({ ...form, text }));

      const res = await (
        await client<ResponseBody<boolean>>(`note`, {
          method: 'DELETE',
          body
        })
      ).body.json();

      return res;
    } catch (err) {
      return { messages: [{ message: 'Server error', path: 'error' }] };
    }
  }
} satisfies Actions;
