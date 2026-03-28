import { useCallback, useEffect, useState } from 'react';
import {
	apiClient,
	clearAuthSession,
	flyerService,
	syncAuthFromStorage,
	UnauthorizedError,
	userService,
	videoService,
	websitePreviewService,
} from '@/lib/api';

type ResourceId = 'flyers' | 'users' | 'videos' | 'website-previews';

const RESOURCE_META: Record<ResourceId, { label: string; listFields: string[] }> = {
	flyers: { label: 'Flyers', listFields: ['name', 'event_title', 'is_active'] },
	users: { label: 'Users', listFields: ['name', 'email', 'role', 'is_active'] },
	videos: { label: 'Videos', listFields: ['name', 'url', 'is_active'] },
	'website-previews': {
		label: 'Website previews',
		listFields: ['title', 'url', 'content_type', 'is_active'],
	},
};

const ENTITY = {
	flyers: flyerService,
	users: userService,
	videos: videoService,
	'website-previews': websitePreviewService,
} as const;

type DialogOpen =
	| {
			open: true;
			resourceId: ResourceId;
			mode: 'create' | 'edit';
			id: number | null;
			initial: Record<string, unknown> | null;
	  }
	| { open: false };

function appendBool(fd: FormData, key: string, checked: boolean) {
	fd.append(key, checked ? 'true' : 'false');
}

function validateForm(resourceId: ResourceId, mode: 'create' | 'edit', form: HTMLFormElement): string | null {
	const named = (n: string) => form.elements.namedItem(n);

	if (resourceId === 'flyers' && mode === 'create') {
		const name = named('name');
		const file = named('file');
		if (!(name instanceof HTMLInputElement) || !name.value.trim()) return 'Name is required';
		if (!(file instanceof HTMLInputElement) || !file.files?.[0]) return 'File is required';
	}
	if (resourceId === 'users' && mode === 'create') {
		const name = named('name');
		const email = named('email');
		if (!(name instanceof HTMLInputElement) || !name.value.trim()) return 'Name is required';
		if (!(email instanceof HTMLInputElement) || !email.value.trim()) return 'Email is required';
	}
	if (resourceId === 'videos' && mode === 'create') {
		const name = named('name');
		const url = named('url');
		if (!(name instanceof HTMLInputElement) || !name.value.trim()) return 'Name is required';
		if (!(url instanceof HTMLInputElement) || !url.value.trim()) return 'URL is required';
	}
	if (resourceId === 'website-previews' && mode === 'create') {
		const url = named('url');
		const ct = named('content_type');
		if (!(url instanceof HTMLInputElement) || !url.value.trim()) return 'URL is required';
		if (!(ct instanceof HTMLInputElement) || !ct.value.trim()) return 'Content type is required';
	}
	return null;
}

function readForm(resourceId: ResourceId, form: HTMLFormElement): FormData {
	const fd = new FormData();
	const get = (n: string) => form.elements.namedItem(n);
	const textOptional = (name: string) => {
		const el = get(name);
		if (el instanceof HTMLInputElement || el instanceof HTMLTextAreaElement) {
			const v = el.value.trim();
			if (v) fd.append(name, v);
		}
	};

	switch (resourceId) {
		case 'flyers': {
			const nameEl = get('name');
			if (nameEl instanceof HTMLInputElement && nameEl.value.trim()) fd.append('name', nameEl.value.trim());
			const fileEl = get('file');
			if (fileEl instanceof HTMLInputElement && fileEl.files?.[0]) fd.append('file', fileEl.files[0]);
			textOptional('event_title');
			textOptional('short_info');
			textOptional('description');
			const c = get('is_active');
			if (c instanceof HTMLInputElement) appendBool(fd, 'is_active', c.checked);
			break;
		}
		case 'users': {
			const nameEl = get('name');
			if (nameEl instanceof HTMLInputElement && nameEl.value.trim()) fd.append('name', nameEl.value.trim());
			const emailEl = get('email');
			if (emailEl instanceof HTMLInputElement && emailEl.value.trim()) fd.append('email', emailEl.value.trim());
			textOptional('phone');
			textOptional('role');
			textOptional('initials');
			textOptional('description');
			const skillsEl = get('skills');
			if (skillsEl instanceof HTMLTextAreaElement) {
				const parts = skillsEl.value
					.split(/[\n,]+/)
					.map((s) => s.trim())
					.filter(Boolean);
				for (const p of parts) fd.append('skills', p);
			}
			const av = get('avatar');
			if (av instanceof HTMLInputElement && av.files?.[0]) fd.append('avatar', av.files[0]);
			const c = get('is_active');
			if (c instanceof HTMLInputElement) appendBool(fd, 'is_active', c.checked);
			break;
		}
		case 'videos': {
			const nameEl = get('name');
			if (nameEl instanceof HTMLInputElement && nameEl.value.trim()) fd.append('name', nameEl.value.trim());
			const urlEl = get('url');
			if (urlEl instanceof HTMLInputElement && urlEl.value.trim()) fd.append('url', urlEl.value.trim());
			textOptional('event_title');
			textOptional('short_info');
			textOptional('description');
			const th = get('thumbnail');
			if (th instanceof HTMLInputElement && th.files?.[0]) fd.append('thumbnail', th.files[0]);
			const c = get('is_active');
			if (c instanceof HTMLInputElement) appendBool(fd, 'is_active', c.checked);
			break;
		}
		case 'website-previews': {
			const urlEl = get('url');
			if (urlEl instanceof HTMLInputElement && urlEl.value.trim()) fd.append('url', urlEl.value.trim());
			const ctEl = get('content_type');
			if (ctEl instanceof HTMLInputElement && ctEl.value.trim()) fd.append('content_type', ctEl.value.trim());
			textOptional('title');
			textOptional('short_info');
			textOptional('description');
			textOptional('favicon_url');
			const im = get('image');
			if (im instanceof HTMLInputElement && im.files?.[0]) fd.append('image', im.files[0]);
			const c = get('is_active');
			if (c instanceof HTMLInputElement) appendBool(fd, 'is_active', c.checked);
			break;
		}
	}
	return fd;
}

function rowSummary(resourceId: ResourceId, item: Record<string, unknown>): string {
	if (resourceId === 'users') return String(item.name ?? item.email ?? item.id);
	if (resourceId === 'videos') return String(item.name ?? item.url ?? item.id);
	if (resourceId === 'flyers') return String(item.name ?? item.title ?? item.id);
	return String(item.title ?? item.url ?? item.id);
}

function formatCell(v: unknown): string {
	if (typeof v === 'boolean') return v ? 'yes' : 'no';
	if (v == null) return '—';
	if (typeof v === 'object') return JSON.stringify(v);
	return String(v);
}

function inputClass() {
	return 'mt-1 w-full rounded-lg border border-white/20 bg-black/50 px-3 py-2 text-sm text-white outline-none focus:border-nelf-pink/70';
}

function EntityFields({
	resourceId,
	mode,
	initial,
}: {
	resourceId: ResourceId;
	mode: 'create' | 'edit';
	initial: Record<string, unknown> | null;
}) {
	const d = initial ?? {};
	const activeDefault = d.is_active !== false;

	switch (resourceId) {
		case 'flyers':
			return (
				<>
					<div>
						<label className="block text-sm text-white/75" htmlFor="name">
							Name
						</label>
						<input className={inputClass()} id="name" name="name" required={mode === 'create'} defaultValue={String(d.name ?? '')} />
					</div>
					<div>
						<label className="block text-sm text-white/75" htmlFor="file">
							File
						</label>
						<input
							className="mt-1 w-full text-sm text-white/70 file:mr-3 file:rounded-md file:border-0 file:bg-white/10 file:px-3 file:py-1.5 file:text-white"
							id="file"
							name="file"
							type="file"
							required={mode === 'create'}
						/>
						{mode === 'edit' ? <p className="mt-1 text-xs text-white/45">Leave empty to keep current file.</p> : null}
					</div>
					<div>
						<label className="block text-sm text-white/75" htmlFor="event_title">
							Event title
						</label>
						<input className={inputClass()} id="event_title" name="event_title" defaultValue={String(d.event_title ?? '')} />
					</div>
					<div>
						<label className="block text-sm text-white/75" htmlFor="short_info">
							Short info
						</label>
						<textarea className={inputClass()} id="short_info" name="short_info" rows={3} defaultValue={String(d.short_info ?? '')} />
					</div>
					<div>
						<label className="block text-sm text-white/75" htmlFor="description">
							Description
						</label>
						<textarea className={inputClass()} id="description" name="description" rows={3} defaultValue={String(d.description ?? '')} />
					</div>
					<div className="flex items-center gap-2">
						<input className="h-4 w-4 accent-nelf-pink" type="checkbox" id="is_active" name="is_active" defaultChecked={activeDefault} />
						<label htmlFor="is_active" className="text-sm text-white/80">
							Active
						</label>
					</div>
				</>
			);
		case 'users': {
			const skillsVal = Array.isArray(d.skills) ? (d.skills as string[]).join(', ') : String(d.skills ?? '');
			return (
				<>
					<div>
						<label className="block text-sm text-white/75" htmlFor="name">
							Name
						</label>
						<input className={inputClass()} id="name" name="name" required={mode === 'create'} defaultValue={String(d.name ?? '')} />
					</div>
					<div>
						<label className="block text-sm text-white/75" htmlFor="email">
							Email
						</label>
						<input
							className={inputClass()}
							id="email"
							name="email"
							type="email"
							required={mode === 'create'}
							defaultValue={String(d.email ?? '')}
						/>
					</div>
					<div>
						<label className="block text-sm text-white/75" htmlFor="phone">
							Phone
						</label>
						<input className={inputClass()} id="phone" name="phone" defaultValue={String(d.phone ?? '')} />
					</div>
					<div>
						<label className="block text-sm text-white/75" htmlFor="role">
							Role
						</label>
						<input className={inputClass()} id="role" name="role" defaultValue={String(d.role ?? '')} />
					</div>
					<div>
						<label className="block text-sm text-white/75" htmlFor="initials">
							Initials
						</label>
						<input className={inputClass()} id="initials" name="initials" defaultValue={String(d.initials ?? '')} />
					</div>
					<div>
						<label className="block text-sm text-white/75" htmlFor="description">
							Description
						</label>
						<textarea className={inputClass()} id="description" name="description" rows={3} defaultValue={String(d.description ?? '')} />
					</div>
					<div>
						<label className="block text-sm text-white/75" htmlFor="skills">
							Skills
						</label>
						<textarea className={inputClass()} id="skills" name="skills" rows={2} defaultValue={skillsVal} />
						<p className="mt-1 text-xs text-white/45">Comma-separated</p>
					</div>
					<div>
						<label className="block text-sm text-white/75" htmlFor="avatar">
							Avatar
						</label>
						<input
							className="mt-1 w-full text-sm text-white/70 file:mr-3 file:rounded-md file:border-0 file:bg-white/10 file:px-3 file:py-1.5 file:text-white"
							id="avatar"
							name="avatar"
							type="file"
						/>
					</div>
					<div className="flex items-center gap-2">
						<input className="h-4 w-4 accent-nelf-pink" type="checkbox" id="is_active" name="is_active" defaultChecked={activeDefault} />
						<label htmlFor="is_active" className="text-sm text-white/80">
							Active
						</label>
					</div>
				</>
			);
		}
		case 'videos':
			return (
				<>
					<div>
						<label className="block text-sm text-white/75" htmlFor="name">
							Name
						</label>
						<input className={inputClass()} id="name" name="name" required={mode === 'create'} defaultValue={String(d.name ?? '')} />
					</div>
					<div>
						<label className="block text-sm text-white/75" htmlFor="url">
							URL
						</label>
						<input className={inputClass()} id="url" name="url" type="url" required={mode === 'create'} defaultValue={String(d.url ?? '')} />
					</div>
					<div>
						<label className="block text-sm text-white/75" htmlFor="event_title">
							Event title
						</label>
						<input className={inputClass()} id="event_title" name="event_title" defaultValue={String(d.event_title ?? '')} />
					</div>
					<div>
						<label className="block text-sm text-white/75" htmlFor="short_info">
							Short info
						</label>
						<textarea className={inputClass()} id="short_info" name="short_info" rows={3} defaultValue={String(d.short_info ?? '')} />
					</div>
					<div>
						<label className="block text-sm text-white/75" htmlFor="description">
							Description
						</label>
						<textarea className={inputClass()} id="description" name="description" rows={3} defaultValue={String(d.description ?? '')} />
					</div>
					<div>
						<label className="block text-sm text-white/75" htmlFor="thumbnail">
							Thumbnail
						</label>
						<input
							className="mt-1 w-full text-sm text-white/70 file:mr-3 file:rounded-md file:border-0 file:bg-white/10 file:px-3 file:py-1.5 file:text-white"
							id="thumbnail"
							name="thumbnail"
							type="file"
						/>
					</div>
					<div className="flex items-center gap-2">
						<input className="h-4 w-4 accent-nelf-pink" type="checkbox" id="is_active" name="is_active" defaultChecked={activeDefault} />
						<label htmlFor="is_active" className="text-sm text-white/80">
							Active
						</label>
					</div>
				</>
			);
		case 'website-previews':
			return (
				<>
					<div>
						<label className="block text-sm text-white/75" htmlFor="url">
							URL
						</label>
						<input className={inputClass()} id="url" name="url" type="url" required={mode === 'create'} defaultValue={String(d.url ?? '')} />
					</div>
					<div>
						<label className="block text-sm text-white/75" htmlFor="content_type">
							Content type
						</label>
						<input
							className={inputClass()}
							id="content_type"
							name="content_type"
							required={mode === 'create'}
							defaultValue={String(d.content_type ?? '')}
						/>
					</div>
					<div>
						<label className="block text-sm text-white/75" htmlFor="title">
							Title
						</label>
						<input className={inputClass()} id="title" name="title" defaultValue={String(d.title ?? '')} />
					</div>
					<div>
						<label className="block text-sm text-white/75" htmlFor="short_info">
							Short info
						</label>
						<textarea className={inputClass()} id="short_info" name="short_info" rows={3} defaultValue={String(d.short_info ?? '')} />
					</div>
					<div>
						<label className="block text-sm text-white/75" htmlFor="description">
							Description
						</label>
						<textarea className={inputClass()} id="description" name="description" rows={3} defaultValue={String(d.description ?? '')} />
					</div>
					<div>
						<label className="block text-sm text-white/75" htmlFor="favicon_url">
							Favicon URL
						</label>
						<input className={inputClass()} id="favicon_url" name="favicon_url" type="url" defaultValue={String(d.favicon_url ?? '')} />
					</div>
					<div>
						<label className="block text-sm text-white/75" htmlFor="image">
							Image
						</label>
						<input
							className="mt-1 w-full text-sm text-white/70 file:mr-3 file:rounded-md file:border-0 file:bg-white/10 file:px-3 file:py-1.5 file:text-white"
							id="image"
							name="image"
							type="file"
						/>
					</div>
					<div className="flex items-center gap-2">
						<input className="h-4 w-4 accent-nelf-pink" type="checkbox" id="is_active" name="is_active" defaultChecked={activeDefault} />
						<label htmlFor="is_active" className="text-sm text-white/80">
							Active
						</label>
					</div>
				</>
			);
		default:
			return null;
	}
}

const RESOURCE_IDS = ['flyers', 'users', 'videos', 'website-previews'] as const;

function parseResourceId(raw: string): ResourceId {
	if ((RESOURCE_IDS as readonly string[]).includes(raw)) {
		return raw as ResourceId;
	}
	return 'flyers';
}

/** Interactive panel only: toolbar, data table, modals. Shell (background, sidebar, header) lives in Astro. */
export function AdminDashboardPanel(props: { resource: string }) {
	const resource = parseResourceId(props.resource);
	const [items, setItems] = useState<Record<string, unknown>[]>([]);
	const [loadError, setLoadError] = useState('');
	const [loading, setLoading] = useState(false);
	const [dialog, setDialog] = useState<DialogOpen>({ open: false });
	const [dialogError, setDialogError] = useState('');
	const [deleteId, setDeleteId] = useState<number | null>(null);

	const goLogin = useCallback(() => {
		clearAuthSession();
		globalThis.location.replace('/admin/login');
	}, []);

	useEffect(() => {
		syncAuthFromStorage();
		if (!apiClient.getAuthToken()) {
			goLogin();
		}
	}, [goLogin]);

	useEffect(() => {
		setDialog({ open: false });
		setDeleteId(null);
		setDialogError('');
	}, [resource]);

	const loadList = useCallback(async () => {
		setLoading(true);
		setLoadError('');
		try {
			const list = await ENTITY[resource].listAll();
			setItems(list as unknown as Record<string, unknown>[]);
		} catch (e) {
			if (e instanceof UnauthorizedError) {
				goLogin();
				return;
			}
			setLoadError(e instanceof Error ? e.message : 'Failed to load list');
			setItems([]);
		} finally {
			setLoading(false);
		}
	}, [resource, goLogin]);

	useEffect(() => {
		if (!apiClient.getAuthToken()) return;
		void loadList();
	}, [loadList]);

	const meta = RESOURCE_META[resource];
	const fields = ['id', ...meta.listFields.filter((f) => f !== 'id')];

	function openCreate() {
		setDialogError('');
		setDialog({ open: true, resourceId: resource, mode: 'create', id: null, initial: null });
	}

	async function openEdit(id: number) {
		setDialogError('');
		try {
			const one = await ENTITY[resource].getOne(id);
			setDialog({
				open: true,
				resourceId: resource,
				mode: 'edit',
				id,
				initial: one as unknown as Record<string, unknown>,
			});
		} catch (e) {
			if (e instanceof UnauthorizedError) {
				goLogin();
				return;
			}
			setLoadError(e instanceof Error ? e.message : 'Could not load item');
		}
	}

	function closeDialog() {
		setDialog({ open: false });
		setDialogError('');
	}

	async function submitEntity(e: React.FormEvent<HTMLFormElement>) {
		e.preventDefault();
		if (!dialog.open) return;
		const form = e.currentTarget;
		setDialogError('');
		const err = validateForm(dialog.resourceId, dialog.mode, form);
		if (err) {
			setDialogError(err);
			return;
		}
		const fd = readForm(dialog.resourceId, form);
		const svc = ENTITY[dialog.resourceId];
		try {
			if (dialog.mode === 'create') {
				await svc.createFormData(fd);
			} else if (dialog.id != null) {
				await svc.updateFormData(dialog.id, fd);
			}
			closeDialog();
			await loadList();
		} catch (err) {
			if (err instanceof UnauthorizedError) {
				goLogin();
				return;
			}
			setDialogError(err instanceof Error ? err.message : 'Save failed');
		}
	}

	async function confirmDelete() {
		if (deleteId == null) return;
		const id = deleteId;
		setDeleteId(null);
		try {
			await ENTITY[resource].removeOne(id);
			await loadList();
		} catch (e) {
			if (e instanceof UnauthorizedError) {
				goLogin();
				return;
			}
			setLoadError(e instanceof Error ? e.message : 'Delete failed');
		}
	}

	const dialogKey = dialog.open ? `${dialog.resourceId}-${dialog.mode}-${dialog.id ?? 'new'}` : 'closed';

	return (
		<>
			<div className="space-y-4">
				<div className="flex flex-wrap items-center justify-between gap-3">
					<div>
						<h2 className="text-2xl font-semibold">{meta.label}</h2>
						<p className="mt-1 text-sm text-white/55">
							Create, edit, and remove {meta.label.toLowerCase()} via the API.
						</p>
					</div>
					<div className="flex gap-2">
						<button
							type="button"
							onClick={() => void loadList()}
							className="rounded-lg border border-white/25 px-4 py-2 text-sm hover:bg-white/5"
						>
							Refresh
						</button>
						<button
							type="button"
							onClick={() => void openCreate()}
							className="rounded-lg border border-white/70 bg-white px-4 py-2 text-sm font-medium text-gray-900 transition-transform hover:-translate-y-0.5"
						>
							Add new
						</button>
					</div>
				</div>

				{loadError ? <p className="text-sm text-red-400">{loadError}</p> : null}
				{loading ? <p className="text-sm text-white/50">Loading…</p> : null}

				<div className="overflow-x-auto rounded-xl border border-white/10 bg-black/30">
					<table className="min-w-full text-left text-sm">
						<thead className="border-b border-white/10 text-white/60">
							<tr>
								{fields.map((f) => (
									<th key={f} className="px-4 py-3 font-medium">
										{f.replace(/_/g, ' ')}
									</th>
								))}
								<th className="px-4 py-3 text-right font-medium">Actions</th>
							</tr>
						</thead>
						<tbody>
							{items.length === 0 && !loading ? (
								<tr>
									<td colSpan={fields.length + 1} className="px-4 py-8 text-center text-white/45">
										No items yet.
									</td>
								</tr>
							) : (
								items.map((item) => (
									<tr key={String(item.id)} className="border-b border-white/5 hover:bg-white/[0.03]">
										{fields.map((f) => (
											<td
												key={f}
												className="max-w-[14rem] truncate px-4 py-3 text-white/85"
												title={formatCell(item[f])}
											>
												{formatCell(item[f])}
											</td>
										))}
										<td className="space-x-2 whitespace-nowrap px-4 py-3 text-right">
											<button
												type="button"
												className="text-xs font-medium text-nelf-pink hover:underline"
												onClick={() => void openEdit(Number(item.id))}
											>
												Edit
											</button>
											<button
												type="button"
												className="text-xs font-medium text-white/50 hover:text-red-400"
												onClick={() => setDeleteId(Number(item.id))}
											>
												Delete
											</button>
										</td>
									</tr>
								))
							)}
						</tbody>
					</table>
				</div>
			</div>

			{dialog.open ? (
				<div className="fixed inset-0 z-50 flex items-center justify-center bg-black/70 p-4" role="presentation" onClick={closeDialog}>
					<div
						className="flex max-h-[90vh] w-full max-w-lg flex-col overflow-y-auto rounded-xl border border-white/15 bg-neutral-950 text-white"
						role="dialog"
						aria-modal
						onClick={(ev) => ev.stopPropagation()}
					>
						<form key={dialogKey} onSubmit={(ev) => void submitEntity(ev)} className="flex flex-col">
							<div className="flex items-start justify-between gap-3 border-b border-white/10 px-5 py-4">
								<div>
									<p className="text-xs uppercase tracking-widest text-white/50">{RESOURCE_META[dialog.resourceId].label}</p>
									<h2 className="mt-1 text-xl font-semibold">
										{dialog.mode === 'create' ? 'New item' : `Edit · ${rowSummary(dialog.resourceId, dialog.initial ?? {})}`}
									</h2>
								</div>
								<button type="button" className="rounded-md p-2 text-white/60 hover:bg-white/10 hover:text-white" onClick={closeDialog}>
									✕
								</button>
							</div>
							<div className="flex-1 space-y-4 px-5 py-4">
								<EntityFields resourceId={dialog.resourceId} mode={dialog.mode} initial={dialog.initial} />
							</div>
							{dialogError ? (
								<p className="mx-5 text-sm text-red-400" role="alert">
									{dialogError}
								</p>
							) : null}
							<div className="flex justify-end gap-2 border-t border-white/10 px-5 py-4">
								<button type="button" className="rounded-lg border border-white/25 px-4 py-2 text-sm hover:bg-white/5" onClick={closeDialog}>
									Cancel
								</button>
								<button
									type="submit"
									className="rounded-lg border border-white/70 bg-white px-4 py-2 text-sm font-medium text-gray-900 transition-transform hover:-translate-y-0.5"
								>
									Save
								</button>
							</div>
						</form>
					</div>
				</div>
			) : null}

			{deleteId != null ? (
				<div className="fixed inset-0 z-50 flex items-center justify-center bg-black/70 p-4">
					<div className="max-w-sm rounded-xl border border-white/15 bg-neutral-950 p-6 text-white">
						<p className="text-sm text-white/85">Delete this item? This cannot be undone.</p>
						<div className="mt-5 flex justify-end gap-2">
							<button
								type="button"
								className="rounded-lg border border-white/25 px-4 py-2 text-sm hover:bg-white/5"
								onClick={() => setDeleteId(null)}
							>
								Cancel
							</button>
							<button
								type="button"
								className="rounded-lg border border-red-500/50 bg-red-950/80 px-4 py-2 text-sm text-red-100 hover:bg-red-900/80"
								onClick={() => void confirmDelete()}
							>
								Delete
							</button>
						</div>
					</div>
				</div>
			) : null}
		</>
	);
}
