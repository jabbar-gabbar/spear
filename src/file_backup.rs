fn run(){
    list_upload();

    // run(settings: Settings) -> Result<bool, Err>;

    // Settings::default() to read settings file
    // for each backup [inv_path, source_path, s3_bucket_name] in settings
    //  get an inventory.list(read_to_string_impl) using inv_path (done)
    //  source.list(source_path) get source files: Vec<string>
    //  calculate filter_upload.filter() from both (unit test)
    //  for f in uploads
    //      s3.upload(abs_f_path: string)
    //          create key name prefix + file name. Prefix = file path.substring(source_path - file_path.substring(filepath.lastindex('/'))) //main/dir_1/dir_2/hi.jpg
    //          aws_sdk.s3_client.upload(abs_f_path, key_name) ->Result<boolean,Error>
    //      if success, add to the list of completed list (unit test)
    //      else log error in console
    //  inventory.append(append_impl, new_content) the inventory file with the completed inv list 
}