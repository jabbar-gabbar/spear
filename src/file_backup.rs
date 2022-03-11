fn run(){
    list_upload();

    // run(settings: Settings) -> Result<bool, Err>;

    //for a dir -> inv_path, source_path, s3_bucket_name
    //  get an inventory_list from inv_path
    //  get source_list from source_path
    //  calculate upload_list from both (unit test)
    //  for f in upload_files
    //      contact s3 and upload it 
    //          if success, add to the list of completed list (unit test)
    //          else log error in console (unit test)
    //  append the inventory file with the completed inv list 
}