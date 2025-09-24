import type { CreateUserRequest } from '../types/user';

export const validateEmail = (email: string): boolean => {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return emailRegex.test(email);
};

export const validateUsername = (username: string): boolean => {
  return username.length >= 3 && username.length <= 20;
};

export const validateUserRequest = (request: CreateUserRequest): string[] => {
  const errors: string[] = [];

  if (!validateUsername(request.username)) {
    errors.push('Username must be between 3 and 20 characters');
  }

  if (!validateEmail(request.email)) {
    errors.push('Invalid email format');
  }

  return errors;
};
