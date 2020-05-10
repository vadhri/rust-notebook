// in src/users.js
import React from 'react';
import { List, ReferenceField, Datagrid, TextField, EmailField, UrlField } from 'react-admin';

export const UserList = props => (
    <List {...props}>
        <Datagrid rowClick="edit">
            <TextField label="ID" source="id" />
            <TextField source="given_name" />
            <TextField source="last_name" />
            <EmailField source="email" />
            <TextField source="city" />
            <TextField source="pincode" />
        </Datagrid>
    </List>
);
