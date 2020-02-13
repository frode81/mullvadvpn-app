#pragma once

#include "ifirewallrule.h"

namespace rules
{

class BlockDns : public IFirewallRule
{
public:

	// TODO: allow relay IP exemption

	bool apply(IObjectInstaller &objectInstaller) override;
};

}
