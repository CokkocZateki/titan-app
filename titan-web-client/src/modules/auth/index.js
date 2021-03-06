import { createStateReducer } from 'titan/lib/redux/stateReducer';
import authReducer from 'titan/reducers/authReducer';
import { EmptyDarkLayout } from 'titan/layouts';
import LoginScene from './login';
import LogoutScene from './logout';
import {
  ROUTE_TYPE_AUTHENTICATED,
  ROUTE_TYPE_UNAUTHENTICATED
} from '../../lib/routing';

export default function () {
  return {
    config: {
      login_path: '/auth/login',
      logout_path: '/auth/logout',
      woltlab: {
        login_url: 'https://clanunknownsoldiers.com/login'
      }
    },
    name: 'auth',
    reducer: createStateReducer({
      session: authReducer
    }),
    routes: [
      {
        layout: EmptyDarkLayout,
        path: '/auth/login',
        scene: LoginScene,
        type: ROUTE_TYPE_UNAUTHENTICATED
      },
      {
        layout: EmptyDarkLayout,
        path: '/auth/logout',
        scene: LogoutScene,
        type: ROUTE_TYPE_AUTHENTICATED
      }
    ]
  };
}
