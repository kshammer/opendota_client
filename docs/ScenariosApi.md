# \ScenariosApi

All URIs are relative to *http://api.opendota.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**scenarios_item_timings_get**](ScenariosApi.md#scenarios_item_timings_get) | **GET** /scenarios/itemTimings | GET /scenarios/itemTimings
[**scenarios_lane_roles_get**](ScenariosApi.md#scenarios_lane_roles_get) | **GET** /scenarios/laneRoles | GET /scenarios/laneRoles
[**scenarios_misc_get**](ScenariosApi.md#scenarios_misc_get) | **GET** /scenarios/misc | GET /scenarios/misc



## scenarios_item_timings_get

> Vec<crate::models::InlineResponse20035> scenarios_item_timings_get(item, hero_id)
GET /scenarios/itemTimings

Win rates for certain item timings on a hero for items that cost at least 1400 gold

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item** | Option<**String**> | Filter by item name e.g. \"spirit_vessel\" |  |
**hero_id** | Option<**i32**> | Hero ID |  |

### Return type

[**Vec<crate::models::InlineResponse20035>**](inline_response_200_35.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scenarios_lane_roles_get

> Vec<crate::models::InlineResponse20036> scenarios_lane_roles_get(lane_role, hero_id)
GET /scenarios/laneRoles

Win rates for heroes in certain lane roles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lane_role** | Option<**String**> | Filter by lane role 1-4 (Safe, Mid, Off, Jungle) |  |
**hero_id** | Option<**i32**> | Hero ID |  |

### Return type

[**Vec<crate::models::InlineResponse20036>**](inline_response_200_36.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scenarios_misc_get

> Vec<crate::models::InlineResponse20037> scenarios_misc_get(scenario)
GET /scenarios/misc

Miscellaneous team scenarios

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scenario** | Option<**String**> | pos_chat_1min,neg_chat_1min,courier_kill,first_blood |  |

### Return type

[**Vec<crate::models::InlineResponse20037>**](inline_response_200_37.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

