// =============================================================================
// Project:        Nelf
// Description:    API configuration with configurable base URL
// =============================================================================

export interface ApiConfig {
  baseUrl: string;
  timeout?: number;
  headers?: Record<string, string>;
}

class ApiClient {
  private config: ApiConfig;
  private authToken: string | null = null;

  constructor(config: ApiConfig) {
    this.config = {
      timeout: 100000,
      headers: {
        'Content-Type': 'application/json',
      },
      ...config,
    };
  }

  setAuthToken(token: string | null): void {
    this.authToken = token;
  }

  getAuthToken(): string | null {
    return this.authToken;
  }

  getConfig(): ApiConfig {
    return { ...this.config };
  }

  updateConfig(newConfig: Partial<ApiConfig>): void {
    this.config = { ...this.config, ...newConfig };
  }

  getBaseUrl(): string {
    return this.config.baseUrl;
  }

  getHeaders(): Record<string, string> {
    const headers = { ...this.config.headers };
    if (this.authToken) {
      headers['Authorization'] = `Bearer ${this.authToken}`;
    }
    return headers;
  }

  getTimeout(): number {
    return this.config.timeout || 100000;
  }
}

// Default configuration — base URL includes /api (see services: flyers → …/api/flyers)
const defaultConfig: ApiConfig = {
  baseUrl: import.meta.env.PUBLIC_API_BASE_URL || 'http://localhost:3000/api',
  timeout: 100000,
  headers: {
    'Content-Type': 'application/json',
  },
};

// Create and export singleton instance
export const apiClient = new ApiClient(defaultConfig);

// Export configuration helper
export const configureApi = (config: Partial<ApiConfig>): void => {
  apiClient.updateConfig(config);
};
