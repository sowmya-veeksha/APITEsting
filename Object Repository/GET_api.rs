<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET_api</name>
   <tag></tag>
   <elementGuidId>ceb0aca2-3915-43b4-bd43-3fe234c619a1</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://${url}?title=${titlevalue}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'localhost:3000/posts'</defaultValue>
      <description></description>
      <id>aed0cbd1-382d-4386-88fa-8d1e0422c406</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>'Sowmya%20M'</defaultValue>
      <description></description>
      <id>9857b59f-0c23-449f-9b1a-cbaf89e3a3b3</id>
      <masked>false</masked>
      <name>authorvalue</name>
   </variables>
   <variables>
      <defaultValue>'POST2_updated'</defaultValue>
      <description></description>
      <id>e3eec015-d834-466d-8035-a3d6f5598ca3</id>
      <masked>false</masked>
      <name>titlevalue</name>
   </variables>
   <variables>
      <defaultValue>'title'</defaultValue>
      <description></description>
      <id>a6873cee-c702-4586-8572-63363fc32168</id>
      <masked>false</masked>
      <name>title</name>
   </variables>
   <variables>
      <defaultValue>'author'</defaultValue>
      <description></description>
      <id>616be0b7-f1fc-4a6f-98f9-264210e6a82d</id>
      <masked>false</masked>
      <name>author</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>b28e6716-37d9-4a21-b092-3f1d9a083cca</id>
      <masked>false</masked>
      <name>responseGET</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyElementPropertyValue(response, '[0].author', 'Sowmya M')
WS.verifyElementPropertyValue(response, '[0].title', 'POST2_updated')
WS.verifyElementPropertyValue(response, '[0].id', '1')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
