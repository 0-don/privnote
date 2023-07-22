import type { Messages, Note, ResponseBody } from '$lib/@types';
import { NoteSchema } from '$lib/schemas/note.schema';
import type { PageServerLoad } from './$types';
import type { Actions } from '@sveltejs/kit';
import { z } from 'zod';
import { COOKIE } from '$lib/utils/server/constants';
import { client, getCsrfToken, messageFactory } from '$lib/utils/server';
import { deepMerge } from '$lib/utils';

export const load = (async (options): Promise<ResponseBody> => await getCsrfToken(options)) satisfies PageServerLoad;

export const actions = {
  default: async ({ request, cookies }): Promise<ResponseBody> => {
    const form = Object.fromEntries(await request.formData());
    try {
      const text = cookies.get(COOKIE);

      const data = NoteSchema.parse(form);

      const response = await (
        await client<ResponseBody<Note>>('note', {
          method: 'POST',
          body: JSON.stringify({ ...data, text })
        })
      ).body.json();

      const note = response.data as Note;

      return deepMerge(response, { messages: messageFactory(note) });
    } catch (err) {
      if (err instanceof z.ZodError) {
        return {
          messages: [
            ...(err.issues.map(({ message, path }) => ({
              message: message,
              path: path.join('.'),
              value: form?.[path.join('.')] || ''
            })) as Messages[]),
            ...messageFactory(form)
          ]
        };
      } else {
        return {
          messages: [
            {
              message: JSON.stringify(err),
              path: 'error'
            },
            ...messageFactory(form)
          ]
        };
      }
    }
  }
} satisfies Actions;
