// =============================================================================
// Project:        Nelf
// Description:    Auth — POST /auth/login (relative to apiClient base, includes /api)
// =============================================================================

import { apiClient } from './config';

export const ADMIN_TOKEN_STORAGE_KEY = 'nelf_access_token';

export interface LoginRequest {
	email: string;
	password: string;
}

export interface AuthResponse {
	access_token: string;
	token_type: string;
}

export async function loginRequest(credentials: LoginRequest): Promise<AuthResponse> {
	const base = apiClient.getBaseUrl().replace(/\/$/, '');
	const url = `${base}/auth/login`;
	const res = await fetch(url, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json', Accept: 'application/json' },
		body: JSON.stringify(credentials),
	});
	if (res.status === 401) {
		throw new Error('Invalid email or password.');
	}
	if (!res.ok) {
		const err = await res.json().catch(() => ({}));
		const msg = typeof (err as { message?: string }).message === 'string' ? (err as { message: string }).message : 'Sign in failed.';
		throw new Error(msg);
	}
	return res.json() as Promise<AuthResponse>;
}

export function persistAuthSession(token: string): void {
	apiClient.setAuthToken(token);
	if (globalThis.localStorage) {
		localStorage.setItem(ADMIN_TOKEN_STORAGE_KEY, token);
	}
}

export function clearAuthSession(): void {
	apiClient.setAuthToken(null);
	if (globalThis.localStorage) {
		localStorage.removeItem(ADMIN_TOKEN_STORAGE_KEY);
	}
}

/** Restore Bearer token on apiClient from localStorage (call on admin pages / before API calls). */
export function syncAuthFromStorage(): void {
	if (!globalThis.localStorage) return;
	apiClient.setAuthToken(localStorage.getItem(ADMIN_TOKEN_STORAGE_KEY));
}

export function isAuthenticated(): boolean {
	return !!apiClient.getAuthToken();
}
