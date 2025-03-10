# neutron-db-opa-proxy

Small proxy to expose limited attributes of OpenStack Neutron resources
necessary for the integration with the OpenPolicyAgent.

Neutron implements custom oslo.policy checks requiring access to certain
attributes of certain resources. It usually deals with them as a so-called
OwnerCheck that i.e. verifies that parent resource of the resource that the
user is currently trying to create/modify belongs to the same project (parent
network of a subnet belongs to the same project). The check itself is inside of
the Neutron code so when it is desired to delegate authorization decisions to
the external system (Open Policy Agent) it is required to have access to the
mentioned attributes. It is undesired to give access to the database directly
(even in the read only mode). Neither is the OpenPolicyAgent by default capable
of accessing DB. So this project comes as a tiny proxy exposing limited
attributes through REST API allowing OPA accessing it also at the same time
relying on the HTTP caching.
