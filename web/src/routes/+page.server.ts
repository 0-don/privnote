/* eslint-disable @typescript-eslint/ban-ts-comment */
import type { CreateNoteResponse } from '$lib/@types';
import { client } from '$lib/utils/client';
import { COOKIE_SERIALIZE_OPTIONS } from '$lib/utils/constants';
import { NoteSchema } from '$lib/utils/schemas/note.schema';
import type { PageServerLoad } from './$types';

import type { Actions } from '@sveltejs/kit';
// @ts-ignore
import { error } from 'console';
import { z } from 'zod';

export const load = (async ({ cookies }) => {
  try {
    const { text } = (await (
      await client('auth/captcha', {
        method: 'GET'
      })
    ).body.json()) as { text: string; id: number };

    cookies.set('captcha', text, COOKIE_SERIALIZE_OPTIONS);

    return {
      captcha: text
    };
  } catch (error) {
    return [{ message: 'Server error', path: 'error' }];
  }
}) satisfies PageServerLoad;

export const actions = {
  default: async ({ request }): Promise<CreateNoteResponse[]> => {
    try {
      const form = Object.fromEntries(await request.formData());

      const data = NoteSchema.parse(form);

      const message = await (
        await client('note', {
          method: 'POST',
          body: JSON.stringify(data)
        })
      ).body.json();

      return [{ message, path: 'ok' }];
    } catch (err) {
      if (err instanceof z.ZodError) {
        return err.issues.map(({ message, path }) => ({
          message: message,
          path: path.join('.')
        })) as CreateNoteResponse[];
      } else {
        error(err);
        return [{ message: 'Unknown error', path: 'error' }];
      }
    }
  }
} satisfies Actions;
