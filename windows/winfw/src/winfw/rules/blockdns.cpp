#include "stdafx.h"
#include "blockdns.h"
#include "winfw/mullvadguids.h"
#include "libwfp/filterbuilder.h"
#include "libwfp/conditionbuilder.h"
#include "libwfp/conditions/conditionport.h"

using namespace wfp::conditions;

namespace
{

constexpr uint16_t DNS_PORT = 53;

} // anonymous namespace

namespace rules
{

bool BlockDns::apply(IObjectInstaller &objectInstaller)
{
	//
	// Block all outbound DNS traffic (IPv4)
	//

	wfp::FilterBuilder filterBuilder;

	filterBuilder
		.provider(MullvadGuids::Provider())
		.description(L"This filter is part of a rule that blocks outbound DNS traffic")
		.sublayer(MullvadGuids::SublayerWhitelist())
		.weight(wfp::FilterBuilder::WeightClass::Max)
		.key(MullvadGuids::FilterBlockDns_Outbound_Ipv4())
		.name(L"Restrict outbound DNS traffic (IPv4)")
		.layer(FWPM_LAYER_ALE_AUTH_CONNECT_V4)
		.block();

	{
		wfp::ConditionBuilder conditionBuilder(FWPM_LAYER_ALE_AUTH_CONNECT_V4);
		conditionBuilder.add_condition(ConditionPort::Remote(DNS_PORT));

		if (!objectInstaller.addFilter(filterBuilder, conditionBuilder))
		{
			return false;
		}
	}

	//
	// Block all outbound DNS traffic (IPv6)
	//

	filterBuilder
		.key(MullvadGuids::FilterBlockDns_Outbound_Ipv6())
		.name(L"Restrict outbound DNS traffic (IPv6)")
		.layer(FWPM_LAYER_ALE_AUTH_CONNECT_V6)
		.block();

	{
		wfp::ConditionBuilder conditionBuilder(FWPM_LAYER_ALE_AUTH_CONNECT_V6);
		conditionBuilder.add_condition(ConditionPort::Remote(DNS_PORT));

		if (!objectInstaller.addFilter(filterBuilder, conditionBuilder))
		{
			return false;
		}
	}

	return true;
}

}
