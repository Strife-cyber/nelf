// =============================================================================
// Project:        Nelf
// Description:    API services exports
// =============================================================================

// Configuration
export { apiClient, configureApi, type ApiConfig } from './config';

export { UnauthorizedError } from './errors';

export {
	loginRequest,
	persistAuthSession,
	clearAuthSession,
	syncAuthFromStorage,
	isAuthenticated,
	ADMIN_TOKEN_STORAGE_KEY,
	type LoginRequest,
	type AuthResponse,
} from './auth-service';

// Base CRUD service
export { BaseCrudService, type CrudService } from './base-crud';

// Individual services
export { userService } from './services/user-service';
export { videoService } from './services/video-service';
export { flyerService } from './services/flyer-service';
export { websitePreviewService } from './services/website-preview-service';

// Service classes (for extending if needed)
export { UserService } from './services/user-service';
export { VideoService } from './services/video-service';
export { FlyerService } from './services/flyer-service';
export { WebsitePreviewService } from './services/website-preview-service';
