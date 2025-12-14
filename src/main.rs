use vm_xml_tool::{Domain, read_vm_config, write_vm_config};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 读取配置
    let mut vm: Domain = read_vm_config("vm1.xml")?;
    println!("Read VM config:{:#?}", vm);
    // 修改配置（例如增加内存）
    vm.memory.value = 2097152; // 2GB
    // 修改磁盘路径
    if let Some(disk) = vm
        .devices
        .disk
        .as_mut()
        .and_then(|disks| disks.iter_mut().find(|d| d.target.dev == "vda"))
    {
        disk.source.file = "/new/path/test.qcow2".to_string();
    }

    // 写回文件
    write_vm_config(&vm, "updated_vm.xml")?;

    println!("VM config updated successfully!");
    Ok(())
}
