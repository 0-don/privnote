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
  note: async ({ request }) => {
    const formData = await request.formData();

    console.log('formData', formData);
  }
};
