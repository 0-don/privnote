import { env } from '$env/dynamic/private';
import { redirect } from '@sveltejs/kit';
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

export const load = () => {
  return {
    contacts
  };
};

export const actions: import('./$types').Actions = {
  createNote: async ({ request: req }) => {
    const formData = await req.formData();

    const { statusCode, headers, trailers, body } = await request(env.ENDPOINT);

    console.log('response received', statusCode);
    console.log('headers', headers);

    // for await (const data of body) {
    //   console.log('data', data);
    // }

    console.log('formData', formData);
    throw redirect(303, '/');
  }
};
