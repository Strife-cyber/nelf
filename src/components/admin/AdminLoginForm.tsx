import { useEffect, useState } from 'react';
import {
	ADMIN_TOKEN_STORAGE_KEY,
	clearAuthSession,
	loginRequest,
	persistAuthSession,
	syncAuthFromStorage,
} from '@/lib/api';

export function AdminLoginForm() {
	const [email, setEmail] = useState('');
	const [password, setPassword] = useState('');
	const [error, setError] = useState('');
	const [busy, setBusy] = useState(false);

	useEffect(() => {
		syncAuthFromStorage();
		if (globalThis.location && localStorage.getItem(ADMIN_TOKEN_STORAGE_KEY)) {
			globalThis.location.replace('/admin');
		}
	}, []);

	async function onSubmit(e: React.FormEvent) {
		e.preventDefault();
		setError('');
		setBusy(true);
		try {
			const res = await loginRequest({ email: email.trim(), password });
			persistAuthSession(res.access_token);
			globalThis.location.replace('/admin');
		} catch (err) {
			clearAuthSession();
			if (err instanceof TypeError) {
				setError('Network error. Is the API running? Check PUBLIC_API_BASE_URL in .env.');
			} else {
				setError(err instanceof Error ? err.message : 'Sign in failed.');
			}
		} finally {
			setBusy(false);
		}
	}

	return (
		<form onSubmit={onSubmit} className="mt-8 space-y-5">
			<div>
				<label htmlFor="email" className="block text-sm font-medium text-white/80">
					Email
				</label>
				<input
					id="email"
					name="email"
					type="email"
					autoComplete="username"
					required
					value={email}
					onChange={(ev) => setEmail(ev.target.value)}
					className="mt-1.5 w-full rounded-lg border border-white/20 bg-black/60 px-3 py-2.5 text-white placeholder:text-white/35 outline-none focus:border-nelf-pink/80 focus:ring-1 focus:ring-nelf-pink/50"
					placeholder="you@example.com"
				/>
			</div>
			<div>
				<label htmlFor="password" className="block text-sm font-medium text-white/80">
					Password
				</label>
				<input
					id="password"
					name="password"
					type="password"
					autoComplete="current-password"
					required
					value={password}
					onChange={(ev) => setPassword(ev.target.value)}
					className="mt-1.5 w-full rounded-lg border border-white/20 bg-black/60 px-3 py-2.5 text-white placeholder:text-white/35 outline-none focus:border-nelf-pink/80 focus:ring-1 focus:ring-nelf-pink/50"
					placeholder="••••••••"
				/>
			</div>
			{error ? (
				<p className="text-sm text-red-400" role="alert">
					{error}
				</p>
			) : null}
			<button
				type="submit"
				disabled={busy}
				className="w-full rounded-lg border border-white/70 bg-white px-4 py-3 text-center text-sm font-medium text-gray-900 transition-all hover:-translate-y-0.5 hover:bg-white/95 disabled:opacity-60"
			>
				{busy ? 'Signing in…' : 'Sign in'}
			</button>
		</form>
	);
}
