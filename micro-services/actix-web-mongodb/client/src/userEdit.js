// in src/users.js
import React from 'react';
import { Edit, SimpleForm, TextInput } from 'react-admin';

export const UserEdit = props => (
    <Edit {...props}>
        <SimpleForm>
          <TextInput source="given_name" />
            <TextInput source="last_name" />
            <TextInput source="email" />
            <TextInput source="city" />
            <TextInput source="pincode" />
        </SimpleForm>
    </Edit>
);
