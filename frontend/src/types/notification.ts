export enum NotificationType {
  ERROR,
  WARN,
  SUCCESS,
}

export type Notification = {
  id: number;
  msg: string;
  type: NotificationType;
  duration: number;
};

export default Notification;
