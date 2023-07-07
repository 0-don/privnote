import { env } from '$env/dynamic/private';
import { client } from '$lib/utils/client';
import { json, redirect } from '@sveltejs/kit';
import { request } from 'undici';

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

export const actions: import('./$types').Actions = {
  createNote: async ({ request: req }) => {
    const formData = await req.formData();

    const data = {
      note: formData.get('note'),
      duration_hours: formData.get('duration_hours'),
      manual_password: formData.get('manual_password'),
      manual_password_confirm: formData.get('manual_password_confirm'),
      notify_email: formData.get('notify_email'),
      notify_ref: formData.get('notify_ref')
    };

    const { statusCode, headers, body } = await client('note', { method: 'POST', body: json(data) });

    console.log('response received', statusCode);
    console.log('headers', headers);

    console.log(body);

    // for await (const data of body) {
    //   console.log('data', data);
    // }

    console.log('formData', formData);
    throw redirect(303, '/');
  }
};
