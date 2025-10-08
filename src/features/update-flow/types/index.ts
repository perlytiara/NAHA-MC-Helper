export interface MinecraftInstance {
  name: string;
  launcher_type: string;
  minecraft_version: string;
  mod_loader: string;
  instance_path: string;
  mod_count: number;
}

export interface UpdateProgress {
  stage: 'scanning' | 'downloading' | 'updating' | 'complete';
  message: string;
  percentage?: number;
}

