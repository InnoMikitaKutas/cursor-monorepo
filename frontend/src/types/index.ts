export interface User {
  id: string;
  name: string;
  username: string;
  email: string;
  phone?: string;
  website?: string;
  address?: Address;
  company?: Company;
  created_at: string;
  updated_at: string;
}

export interface Address {
  id: string;
  user_id: string;
  street: string;
  suite?: string;
  city: string;
  zipcode: string;
  geo?: Geo;
}

export interface Geo {
  lat: number;
  lng: number;
}

export interface Company {
  id: string;
  user_id: string;
  name: string;
  catch_phrase?: string;
  bs?: string;
}

export interface AuthUser {
  id: string;
  name: string;
  email: string;
  created_at: string;
}

export interface AuthResponse {
  token: string;
  user: AuthUser;
}

export interface LoginRequest {
  email: string;
  password: string;
}

export interface RegisterRequest {
  name: string;
  email: string;
  password: string;
}

export interface CreateUserRequest {
  name: string;
  username: string;
  email: string;
  phone?: string;
  website?: string;
  address?: CreateAddressRequest;
  company?: CreateCompanyRequest;
}

export interface CreateAddressRequest {
  street: string;
  suite?: string;
  city: string;
  zipcode: string;
  geo?: CreateGeoRequest;
}

export interface CreateGeoRequest {
  lat: number;
  lng: number;
}

export interface CreateCompanyRequest {
  name: string;
  catch_phrase?: string;
  bs?: string;
} 