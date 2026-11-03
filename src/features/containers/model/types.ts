/**
 * The parts of the Engine's container inspect document the UI reads. The
 * backend passes the document through untouched (see `containers::inspect`).
 */
export interface ContainerInspect {
  Id: string;
  Name: string;
  Created: string;
  Path: string;
  Args: string[];
  RestartCount: number;
  State: {
    Status: string;
    Running: boolean;
    StartedAt: string;
    FinishedAt: string;
    ExitCode: number;
    OOMKilled: boolean;
    Health?: {
      Status: string;
      FailingStreak: number;
      Log?: { Start: string; ExitCode: number; Output: string }[];
    };
  };
  Config: {
    Hostname: string;
    User: string;
    Env: string[] | null;
    Cmd: string[] | null;
    Entrypoint: string[] | null;
    Image: string;
    WorkingDir: string;
    Labels: Record<string, string> | null;
    Tty: boolean;
  };
  HostConfig: {
    RestartPolicy?: { Name: string; MaximumRetryCount: number };
    NetworkMode: string;
    Memory: number;
    NanoCpus: number;
    AutoRemove: boolean;
    Privileged: boolean;
  };
  Mounts: {
    Type: string;
    Name?: string;
    Source: string;
    Destination: string;
    Mode: string;
    RW: boolean;
  }[];
  NetworkSettings: {
    Ports: Record<string, { HostIp: string; HostPort: string }[] | null> | null;
    Networks: Record<
      string,
      { IPAddress: string; Gateway: string; MacAddress: string; Aliases: string[] | null }
    > | null;
  };
}
