// pub struct DataResponse<T> {
//     code: u32,
//     success: bool,
//     data: Option<T>,
//     message: String,
// }
interface DataResponse<T> {
  code: number;
  success: boolean;
  data: T;
  message: string;
}
