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

  constructor(config: ApiConfig) {
    this.config = {
      timeout: 10000,
      headers: {
        'Content-Type': 'application/json',
      },
      ...config,
    };
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
    return { ...this.config.headers };
  }

  getTimeout(): number {
    return this.config.timeout || 10000;
  }
}

// Default configuration
const defaultConfig: ApiConfig = {
  baseUrl: (globalThis as any).import?.meta?.env?.PUBLIC_API_BASE_URL || 'http://localhost:3000/api',
  timeout: 10000,
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
