// in src/users.js
import React from 'react';
import { Create, SimpleForm, TextInput } from 'react-admin';

export const CreateUser = props => (
    <Create {...props}>
        <SimpleForm>
          <TextInput source="given_name" />
            <TextInput source="last_name" />
            <TextInput source="email" />
            <TextInput source="city" />
            <TextInput source="pincode" />
        </SimpleForm>
    </Create>
);
