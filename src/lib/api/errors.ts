// =============================================================================
// Project:        Nelf
// Description:    API errors
// =============================================================================

export class UnauthorizedError extends Error {
	constructor(message = 'Unauthorized') {
		super(message);
		this.name = 'UnauthorizedError';
	}
}
