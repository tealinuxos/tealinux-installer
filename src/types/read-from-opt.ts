/**
 * This type came from /opt/tea-installer/read.json
 * This type is the result type of the getReadFromOpt function
 */ 

export interface ReadFromOpt {
  model: Model
  memory: Memory
  disk: Disk[]
  battery: Battery
  online: Online
  lspci: Lspci
  firmware: string
  desktopEnvironment: DesktopEnvironment
  operatingSystem: OperatingSystem
  kernel: Kernel
  displayServer: DisplayServer
}

export interface Model {
  systemProductName: string
  systemVersion: string
}

export interface Memory {
  capacity: number
  used: number
}

export interface Disk {
  diskPath: string
  size: string
  model: string
  transport: string
  label: string
  uuid: string
  maxPartition: number
  mountpoints: any[]
  partitions: Partition[]
}

export interface Partition {
  partitionPath?: string
  number: string
  start: string
  end: string
  size: string
  typePartisi: string
  typeUuid?: string
  uuid?: string
  name?: string
  filesystem?: string
  mountpoint?: string[]
  flags?: string[]
}

export interface Battery {
  capacity: number
}

export interface Online {
  status: boolean
}

export interface Lspci {
  cpu: string
  vga: string[]
}

export interface DesktopEnvironment {
  name: string
}

export interface OperatingSystem {
  architecture: string
  version: string
  name: string
}

export interface Kernel {
  name: string
  version: string
}

export interface DisplayServer {
  name: string
}
