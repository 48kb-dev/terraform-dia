# terraform-dia
CLI tool which creates diagrams from parsing terraform files.

# Idea

The idea is to create a tool that can read Terraform configuration files and generate visual diagrams representing the infrastructure defined in those files. This can help developers and DevOps engineers better understand the architecture of their infrastructure, identify dependencies, and communicate designs more effectively. Application also imposes a point of view on architecture, which can be useful for standardization and best practices.

# How it is going to work

it reads terraform files and parses the resources, modules, and their relationships. It then generates a visual representation of the infrastructure and converts it to wasm file which can be used in web applications or other platforms that support WebAssembly. The generated diagrams can be exported in various formats such as SVG, PNG, or PDF for easy sharing and documentation.

# CLi API

CLI provides a simple interface for users to specify the input Terraform files and the desired output format. Users can run commands like:

```
terraform-dia generate --input path/to/terraform/files --output path/to/output --name diagram_name --format wasm|svg|png|pdf
```

# How it is going to be used

included to the CI/CD pipeline to automatically generate diagrams whenever Terraform files are updated. This ensures that the diagrams are always up-to-date with the latest infrastructure changes. Additionally, it can be used as a standalone tool for developers to visualize their infrastructure during the development process.
The reult file could be inserted to readme files or other documentation to provide a visual representation of the infrastructure alongside the code.
