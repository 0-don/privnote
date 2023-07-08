import type { CreateNoteResponse } from '$lib/@types';
import { client } from '$lib/utils/client';
import { NoteSchema } from '$lib/utils/schemas/note.schema';
import type { Actions } from '@sveltejs/kit';
import { error } from 'console';
import { z } from 'zod';

const contacts = [
  {
    id: 'de393e6a-1c08-4e6e-9aad-0994fcf0d981',
    name: 'Saul Goodman',
    email: 'saul@example.com',
    company: 'Saul Goodman & Associates',
    job: 'Attorney'
  }
];

export const load = async () => {
  return {
    contacts
  };
};

export const actions = {
  default: async ({ request }): Promise<CreateNoteResponse[]> => {
    try {
      const data = NoteSchema.parse(Object.fromEntries(await request.formData()));

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
