// in src/App.js
import React from 'react';
import { Admin, Resource, EditGuesser } from 'react-admin';
import { UserList } from './users';
import { UserEdit } from './userEdit';
import { CreateUser } from './createUser';

import jsonServerProvider from 'ra-data-json-server';

const dataProvider = jsonServerProvider('http://127.0.0.1:8081/admin');

const App = () => (
    <Admin dataProvider={dataProvider}>
        <Resource name="users" list={UserList}  edit={UserEdit} create={CreateUser}/>
    </Admin>
);

export default App;
