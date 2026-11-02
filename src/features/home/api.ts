import { invokeCommand } from '@/shared/api';
import type { AppInfo } from './types';

export const getAppInfo = () => invokeCommand<AppInfo>('get_app_info');
