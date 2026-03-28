import { useEffect } from 'react';
import { clearAuthSession } from '@/lib/api';

export function AdminLogoutRedirect() {
	useEffect(() => {
		clearAuthSession();
		globalThis.location.replace('/admin/login');
	}, []);
	return (
		<p className="p-8 font-sans text-sm text-white/70">
			Signing out…
		</p>
	);
}
