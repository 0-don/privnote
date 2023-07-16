import type { ResponseBody, Note } from '$lib/@types';
import { client } from '$lib/utils/client';
import { Redirect, redirect } from '@sveltejs/kit';

import type { PageServerLoad } from './$types';

export const load = (async ({ params }): Promise<ResponseBody | Redirect> => {
  if (!params?.id) return redirect(302, '/');

  try {
    const note = await (
      await client<Note>(`note/${params.id}`, {
        method: 'GET'
      })
    ).body.json();

    return { data: note };
  } catch (error) {
    return { messages: [{ message: 'Server error', path: 'error' }] };
  }
}) satisfies PageServerLoad;
