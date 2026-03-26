# Nelf API Client

This directory contains the API client setup and CRUD services for the Nelf project.

## Structure

- `config.ts` - API configuration with configurable base URL
- `base-crud.ts` - Base CRUD service template
- `services/` - Individual CRUD services for each model
- `index.ts` - Main exports

## Usage

### Configuration

```typescript
import { configureApi } from '@/lib/api';

// Configure API base URL
configureApi({
  baseUrl: 'https://your-api-domain.com/api',
  timeout: 15000,
  headers: {
    'Authorization': 'Bearer your-token'
  }
});
```

### Using Services

```typescript
import { userService, videoService, flyerService, websitePreviewService } from '@/lib/api';

// Get all users
const users = await userService.getAll({ page: 1, limit: 10 });

// Create a new user
const newUser = await userService.create({
  name: 'John Doe',
  email: 'john@example.com',
  role: 'admin'
});

// Update a user
const updatedUser = await userService.update(1, {
  name: 'Jane Doe'
});

// Delete a user
await userService.delete(1);
```

### Extending Services

Each service extends the `BaseCrudService` class and can be extended with custom methods:

```typescript
import { BaseCrudService } from '@/lib/api/base-crud';
import type { User, CreateUserRequest, UpdateUserRequest } from '@/types';

export class CustomUserService extends BaseCrudService<User, CreateUserRequest, UpdateUserRequest> {
  constructor() {
    super('users');
  }

  async customMethod(): Promise<any> {
    // Custom implementation
  }
}
```

## Environment Variables

Set the API base URL using environment variables:

```
PUBLIC_API_BASE_URL=https://your-api-domain.com/api
```

If not set, defaults to `http://localhost:3000/api`.
