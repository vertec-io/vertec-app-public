# **BUILT-IN PLUGINS**
---

Vertec provides several useful built-in plugins. Many of them are free to use and some are available with an additional license purchase. For now, all plugins will be made available for free.

These plugins are structured as Vertices within the Vertex-Management-Service architecture. They are the primary way that all vertices are defined. In general a Vertex can be a unit building block of the IIoT Automation Network/Graph. This includes things like PLCs, sensors, devices, and even units of code that can be executed.

The goal behind this plugin architecture is to enable a highly scalable approach to developing software for the network to support, control, monitor, and test vertices in the physical world and the software world. This will enable a large community of developers, manufacturers, and users to enrich the Vertec ecosystem with custom plugins for brand-specific devices, unique custom-made hardware, or custom-developed software to integrate into the framework. In the future this could also enable a marketplace where others may also contribute, and share and sell their plugin developments with others, enabling entreprenuerial activity on top of the Vertec platform.

## Requirements

Each plugin should be structured as a rust module and should implement a specific plugin for handling a specific vertex type with its own . The plugin should implement the PluginApi trait and register itself with the PluginManager.

This directory contains the Rust plugins that can be installed dynamically. Each plugin lives in its own subdirectory, with its own lib.rs and Cargo.toml.