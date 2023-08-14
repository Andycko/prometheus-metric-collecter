terraform {
  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "~> 3.69.0"
    }
  }

  required_version = ">= 1.1.0"
}

provider "azurerm" {
  features {}
}

# Identities & roles
resource "azurerm_user_assigned_identity" "metricscraper-ma" {
  name                = "metricscraper-ma"
  location            = azurerm_resource_group.rg.location
  resource_group_name = azurerm_resource_group.rg.name
}

resource "azurerm_role_assignment" "assign_identity_storage_blob_data_contributor" {
  scope                = azurerm_storage_account.sa.id
  role_definition_name = "Storage Blob Data Contributor"
  principal_id         = azurerm_user_assigned_identity.metricscraper-ma.principal_id
}

resource "azurerm_role_assignment" "assign_identity_container_registry_reader" {
  scope                = azurerm_container_registry.cr.id
  role_definition_name = "AcrPull"
  principal_id         = azurerm_user_assigned_identity.metricscraper-ma.principal_id
}

// Static resources
resource "azurerm_resource_group" "rg" {
  name     = "healthdashboard-rg"
  location = "westus2"
}

resource "azurerm_storage_account" "sa" {
  name                     = "healthdashboard-sa"
  resource_group_name      = azurerm_resource_group.rg.name
  location                 = azurerm_resource_group.rg.location
  account_tier             = "Standard"
  account_replication_type = "LRS"
}

resource "azurerm_container_registry" "cr" {
  location            = azurerm_resource_group.rg.location
  name                = "healthdashboard-cr"
  resource_group_name = azurerm_resource_group.rg.name
  sku                 = ""
}

resource "azurerm_log_analytics_workspace" "metricscraper-law" {
  location            = azurerm_resource_group.rg.location
  name                = "metricscraper-law"
  resource_group_name = azurerm_resource_group.rg.name
}

resource "azurerm_container_app_environment" "metriscraper-cae" {
  location                   = azurerm_resource_group.rg.location
  log_analytics_workspace_id = azurerm_log_analytics_workspace.metricscraper-law.name
  name                       = "metricscraper-cae"
  resource_group_name        = azurerm_resource_group.rg.name
}

resource "azurerm_container_app" "metricscraper-ca" {
  container_app_environment_id = azurerm_container_app_environment.metriscraper-cae.id
  name                         = "metricscraper-ca"
  resource_group_name          = azurerm_resource_group.rg.name
  revision_mode                = "single"
  identity {
    type = "UserAssigned"
    identity_ids = [
      azurerm_user_assigned_identity.metricscraper-ma.id
    ]
  }
}