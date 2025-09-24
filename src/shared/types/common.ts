export interface BaseEntity {
  id: string;
  createdAt: Date;
  updatedAt: Date;
}

export interface ApiResponse<T> {
  data: T;
  success: boolean;
  message?: string;
}

export interface NotificationData {
  type: 'info' | 'success' | 'warning' | 'error' | 'loading';
  message: string;
  title?: string;
}
