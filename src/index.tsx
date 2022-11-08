/* @refresh reload */
import './tailwind.css';
import { render } from 'solid-js/web';

import { App } from './app';

render(() => <App />, document.getElementById('root') as HTMLElement);
